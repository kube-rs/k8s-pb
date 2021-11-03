/// CronJob represents the configuration of a single cron job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJob {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of a cron job, including the schedule.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<CronJobSpec>,
    /// Current status of a cron job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<CronJobStatus>,
}
/// CronJobList is a collection of cron jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of CronJobs.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CronJob>,
}
/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobSpec {
    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    #[prost(string, optional, tag="1")]
    pub schedule: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled
    /// time for any reason.  Missed jobs executions will be counted as failed ones.
    /// +optional
    #[prost(int64, optional, tag="2")]
    pub starting_deadline_seconds: ::core::option::Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    /// Valid values are:
    /// - "Allow" (default): allows CronJobs to run concurrently;
    /// - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet;
    /// - "Replace": cancels currently running job and replaces it with a new one
    /// +optional
    #[prost(string, optional, tag="3")]
    pub concurrency_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// This flag tells the controller to suspend subsequent executions, it does
    /// not apply to already started executions.  Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="4")]
    pub suspend: ::core::option::Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    #[prost(message, optional, tag="5")]
    pub job_template: ::core::option::Option<JobTemplateSpec>,
    /// The number of successful finished jobs to retain. Value must be non-negative integer.
    /// Defaults to 3.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub successful_jobs_history_limit: ::core::option::Option<i32>,
    /// The number of failed finished jobs to retain. Value must be non-negative integer.
    /// Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub failed_jobs_history_limit: ::core::option::Option<i32>,
}
/// CronJobStatus represents the current state of a cron job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag="1")]
    pub active: ::prost::alloc::vec::Vec<super::super::core::v1::ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub last_schedule_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Information when was the last time the job successfully completed.
    /// +optional
    #[prost(message, optional, tag="5")]
    pub last_successful_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
}
/// Job represents the configuration of a single job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of a job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<JobSpec>,
    /// Current status of a job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<JobStatus>,
}
/// JobCondition describes current state of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobCondition {
    /// Type of job condition, Complete or Failed.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition was checked.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_probe_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transit from one status to another.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// (brief) reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// JobList is a collection of jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of Jobs.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Job>,
}
/// JobSpec describes how the job execution will look like.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should
    /// run at any given time. The actual number of pods running in steady state will
    /// be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism),
    /// i.e. when the work left to do is less than max parallelism.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub parallelism: ::core::option::Option<i32>,
    /// Specifies the desired number of successfully finished pods the
    /// job should be run with.  Setting to nil means that the success of any
    /// pod signals the success of all pods, and allows parallelism to have any positive
    /// value.  Setting to 1 means that parallelism is limited to 1 and the success of that
    /// pod signals the success of the job.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub completions: ::core::option::Option<i32>,
    /// Specifies the duration in seconds relative to the startTime that the job
    /// may be continuously active before the system tries to terminate it; value
    /// must be positive integer. If a Job is suspended (at creation or through an
    /// update), this timer will effectively be stopped and reset when the Job is
    /// resumed again.
    /// +optional
    #[prost(int64, optional, tag="3")]
    pub active_deadline_seconds: ::core::option::Option<i64>,
    /// Specifies the number of retries before marking this job failed.
    /// Defaults to 6
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub backoff_limit: ::core::option::Option<i32>,
    /// A label query over pods that should match the pod count.
    /// Normally, the system sets this field for you.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(message, optional, tag="4")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// manualSelector controls generation of pod labels and pod selectors.
    /// Leave `manualSelector` unset unless you are certain what you are doing.
    /// When false or unset, the system pick labels unique to this job
    /// and appends those labels to the pod template.  When true,
    /// the user is responsible for picking unique labels and specifying
    /// the selector.  Failure to pick a unique label may cause this
    /// and other jobs to not function correctly.  However, You may see
    /// `manualSelector=true` in jobs that were created with the old `extensions/v1beta1`
    /// API.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    /// +optional
    #[prost(bool, optional, tag="5")]
    pub manual_selector: ::core::option::Option<bool>,
    /// Describes the pod that will be created when executing a job.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[prost(message, optional, tag="6")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished
    /// execution (either Complete or Failed). If this field is set,
    /// ttlSecondsAfterFinished after the Job finishes, it is eligible to be
    /// automatically deleted. When the Job is being deleted, its lifecycle
    /// guarantees (e.g. finalizers) will be honored. If this field is unset,
    /// the Job won't be automatically deleted. If this field is set to zero,
    /// the Job becomes eligible to be deleted immediately after it finishes.
    /// This field is alpha-level and is only honored by servers that enable the
    /// TTLAfterFinished feature.
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub ttl_seconds_after_finished: ::core::option::Option<i32>,
    /// CompletionMode specifies how Pod completions are tracked. It can be
    /// `NonIndexed` (default) or `Indexed`.
    ///
    /// `NonIndexed` means that the Job is considered complete when there have
    /// been .spec.completions successfully completed Pods. Each Pod completion is
    /// homologous to each other.
    ///
    /// `Indexed` means that the Pods of a
    /// Job get an associated completion index from 0 to (.spec.completions - 1),
    /// available in the annotation batch.kubernetes.io/job-completion-index.
    /// The Job is considered complete when there is one successfully completed Pod
    /// for each index.
    /// When value is `Indexed`, .spec.completions must be specified and
    /// `.spec.parallelism` must be less than or equal to 10^5.
    /// In addition, The Pod name takes the form
    /// `$(job-name)-$(index)-$(random-string)`,
    /// the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// This field is beta-level. More completion modes can be added in the future.
    /// If the Job controller observes a mode that it doesn't recognize, the
    /// controller skips updates for the Job.
    /// +optional
    #[prost(string, optional, tag="9")]
    pub completion_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// Suspend specifies whether the Job controller should create Pods or not. If
    /// a Job is created with suspend set to true, no Pods are created by the Job
    /// controller. If a Job is suspended after creation (i.e. the flag goes from
    /// false to true), the Job controller will delete all active Pods associated
    /// with this Job. Users must design their workload to gracefully handle this.
    /// Suspending a Job will reset the StartTime field of the Job, effectively
    /// resetting the ActiveDeadlineSeconds timer too. Defaults to false.
    ///
    /// This field is beta-level, gated by SuspendJob feature flag (enabled by
    /// default).
    ///
    /// +optional
    #[prost(bool, optional, tag="10")]
    pub suspend: ::core::option::Option<bool>,
}
/// JobStatus represents the current state of a Job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    /// The latest available observations of an object's current state. When a Job
    /// fails, one of the conditions will have type "Failed" and status true. When
    /// a Job is suspended, one of the conditions will have type "Suspended" and
    /// status true; when the Job is resumed, the status of this condition will
    /// become false. When a Job is completed, one of the conditions will have
    /// type "Complete" and status true.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=atomic
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<JobCondition>,
    /// Represents time when the job controller started processing a job. When a
    /// Job is created in the suspended state, this field is not set until the
    /// first time it is resumed. This field is reset every time a Job is resumed
    /// from suspension. It is represented in RFC3339 form and is in UTC.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Represents time when the job was completed. It is not guaranteed to
    /// be set in happens-before order across separate operations.
    /// It is represented in RFC3339 form and is in UTC.
    /// The completion time is only set when the job finishes successfully.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub completion_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The number of actively running pods.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub active: ::core::option::Option<i32>,
    /// The number of pods which reached phase Succeeded.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub succeeded: ::core::option::Option<i32>,
    /// The number of pods which reached phase Failed.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub failed: ::core::option::Option<i32>,
    /// CompletedIndexes holds the completed indexes when .spec.completionMode =
    /// "Indexed" in a text format. The indexes are represented as decimal integers
    /// separated by commas. The numbers are listed in increasing order. Three or
    /// more consecutive numbers are compressed and represented by the first and
    /// last element of the series, separated by a hyphen.
    /// For example, if the completed indexes are 1, 3, 4, 5 and 7, they are
    /// represented as "1,3-5,7".
    /// +optional
    #[prost(string, optional, tag="7")]
    pub completed_indexes: ::core::option::Option<::prost::alloc::string::String>,
    /// UncountedTerminatedPods holds the UIDs of Pods that have terminated but
    /// the job controller hasn't yet accounted for in the status counters.
    ///
    /// The job controller creates pods with a finalizer. When a pod terminates
    /// (succeeded or failed), the controller does three steps to account for it
    /// in the job status:
    /// (1) Add the pod UID to the arrays in this field.
    /// (2) Remove the pod finalizer.
    /// (3) Remove the pod UID from the arrays while increasing the corresponding
    ///     counter.
    ///
    /// This field is alpha-level. The job controller only makes use of this field
    /// when the feature gate PodTrackingWithFinalizers is enabled.
    /// Old jobs might not be tracked using this field, in which case the field
    /// remains null.
    /// +optional
    #[prost(message, optional, tag="8")]
    pub uncounted_terminated_pods: ::core::option::Option<UncountedTerminatedPods>,
}
/// JobTemplateSpec describes the data a Job should have when created from a template
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<JobSpec>,
}
/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't
/// been accounted in Job status counters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UncountedTerminatedPods {
    /// Succeeded holds UIDs of succeeded Pods.
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub succeeded: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Failed holds UIDs of failed Pods.
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub failed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

impl crate::Resource for CronJob {
    const API_VERSION: &'static str = "batch/v1";
    const GROUP: &'static str = "batch";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "CronJob";
    const NAME: &'static str = "cronjobs";
}
impl crate::HasMetadata for CronJob {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for CronJob {
    type Spec = crate::api::batch::v1::CronJobSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for CronJob {
    type Status = crate::api::batch::v1::CronJobStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}


impl crate::Resource for Job {
    const API_VERSION: &'static str = "batch/v1";
    const GROUP: &'static str = "batch";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "Job";
    const NAME: &'static str = "jobs";
}
impl crate::HasMetadata for Job {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for Job {
    type Spec = crate::api::batch::v1::JobSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for Job {
    type Status = crate::api::batch::v1::JobStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for Job {
    type Condition = crate::api::batch::v1::JobCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}

